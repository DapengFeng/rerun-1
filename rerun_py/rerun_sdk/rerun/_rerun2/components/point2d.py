# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Sequence, Tuple, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa

__all__ = ["Point2D", "Point2DArray", "Point2DArrayLike", "Point2DLike", "Point2DType"]


## --- Point2D --- ##


@dataclass
class Point2D:
    """A point in 2D space."""

    x: float
    y: float


Point2DLike = Union[Point2D, npt.NDArray[np.float32], Sequence[float], Tuple[float, float]]

Point2DArrayLike = Union[Point2DLike, Sequence[Point2DLike], npt.NDArray[np.float32], Sequence[float]]


# --- Arrow support ---

from .point2d_ext import Point2DArrayExt  # noqa: E402


class Point2DType(pa.ExtensionType):  # type: ignore[misc]
    def __init__(self: type[pa.ExtensionType]) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct([pa.field("x", pa.float32(), False, {}), pa.field("y", pa.float32(), False, {})]),
            "rerun.point2d",
        )

    def __arrow_ext_serialize__(self: type[pa.ExtensionType]) -> bytes:
        # since we don't have a parameterized type, we don't need extra metadata to be deserialized
        return b""

    @classmethod
    def __arrow_ext_deserialize__(
        cls: type[pa.ExtensionType], storage_type: Any, serialized: Any
    ) -> type[pa.ExtensionType]:
        # return an instance of this subclass given the serialized metadata.
        return Point2DType()

    def __arrow_ext_class__(self: type[pa.ExtensionType]) -> type[pa.ExtensionArray]:
        return Point2DArray


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(Point2DType())


class Point2DArray(pa.ExtensionArray, Point2DArrayExt):  # type: ignore[misc]
    @staticmethod
    def from_similar(data: Point2DArrayLike | None) -> pa.Array:
        if data is None:
            return Point2DType().wrap_array(pa.array([], type=Point2DType().storage_type))
        else:
            return Point2DArrayExt._from_similar(
                data,
                mono=Point2D,
                mono_aliases=Point2DLike,
                many=Point2DArray,
                many_aliases=Point2DArrayLike,
                arrow=Point2DType,
            )
