from __future__ import annotations

from dataclasses import fields
from typing import Any, Iterable

import numpy as np
import numpy.typing as npt
import pyarrow as pa

from .. import RecordingStream, bindings
from ..log import error_utils
from .archetypes import Archetype
from .components import Component, InstanceKeyArray

__all__ = ["log_any"]


EXT_PREFIX = "ext."

ext_component_types: dict[str, Any] = {}


# adapted from rerun.log._add_extension_components
def _add_extension_components(
    instanced: dict[str, Component],
    splats: dict[str, Component],
    ext: dict[str, Any],
    identifiers: npt.NDArray[np.uint64] | None,
) -> None:
    global ext_component_types

    for name, value in ext.items():
        # Don't log empty components
        if value is None:
            continue

        # Add the ext prefix, unless it's already there
        if not name.startswith(EXT_PREFIX):
            name = EXT_PREFIX + name

        np_type, pa_type = ext_component_types.get(name, (None, None))

        try:
            if np_type is not None:
                np_value = np.atleast_1d(np.array(value, copy=False, dtype=np_type))
                pa_value = pa.array(np_value, type=pa_type)
            else:
                np_value = np.atleast_1d(np.array(value, copy=False))
                pa_value = pa.array(np_value)
                ext_component_types[name] = (np_value.dtype, pa_value.type)
        except Exception as ex:
            error_utils._send_warning(
                "Error converting extension data to arrow for component {}. Dropping.\n{}: {}".format(
                    name, type(ex).__name__, ex
                ),
                1,
            )
            continue

        is_splat = (len(np_value) == 1) and (len(identifiers or []) != 1)

        if is_splat:
            splats[name] = pa_value  # noqa
        else:
            instanced[name] = pa_value  # noqa


def _extract_components(entity: Archetype) -> Iterable[tuple[Component, bool]]:
    """Extract the components from an entity, yielding (component, is_primary) tuples."""
    for fld in fields(entity):
        if "component" in fld.metadata:
            yield getattr(entity, fld.name), fld.metadata["component"] == "primary"


def log_any(
    entity_path: str,
    entity: Archetype,
    ext: dict[str, Any] | None = None,
    timeless: bool = False,
    recording: RecordingStream | None = None,
) -> None:
    """
    Log an entity.

    Parameters
    ----------
    entity_path:
        Path to the entity in the space hierarchy.
    entity: Archetype
        The archetype object representing the entity.
    ext:
        Optional dictionary of extension components. See [rerun.log_extension_components][]
    timeless:
        If true, the entity will be timeless (default: False).
    recording:
        Specifies the [`rerun.RecordingStream`][] to use.
        If left unspecified, defaults to the current active data recording, if there is one.
        See also: [`rerun.init`][], [`rerun.set_global_data_recording`][].

    """

    from .. import strict_mode

    if strict_mode():
        if not isinstance(entity, Archetype):
            raise TypeError(f"Expected Archetype, got {type(entity)}")

    instanced: dict[str, Component] = {}
    splats: dict[str, Component] = {}

    # find canonical length of this entity by based on the longest length of any primary component
    archetype_length = max(len(comp) for comp, primary in _extract_components(entity) if primary)

    for comp, primary in _extract_components(entity):
        if primary:
            instanced[comp.extension_name] = comp.storage
        elif len(comp) == 1 and archetype_length > 1:
            splats[comp.extension_name] = comp.storage
        elif len(comp) > 1:
            instanced[comp.extension_name] = comp.storage

    if ext:
        _add_extension_components(instanced, splats, ext, None)

    if splats:
        splats["rerun.instance_key"] = InstanceKeyArray.splat()
        bindings.log_arrow_msg(entity_path, components=splats, timeless=timeless, recording=recording)

    # Always the primary component last so range-based queries will include the other data. See(#1215)
    bindings.log_arrow_msg(entity_path, components=instanced, timeless=timeless, recording=recording)
