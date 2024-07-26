# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/transform_relation.fbs".

# You can extend this class by creating a "TransformRelationExt" class in "transform_relation_ext.py".

from __future__ import annotations

from typing import Literal, Sequence, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
    ComponentBatchMixin,
)

__all__ = [
    "TransformRelation",
    "TransformRelationArrayLike",
    "TransformRelationBatch",
    "TransformRelationLike",
    "TransformRelationType",
]


from enum import Enum


class TransformRelation(Enum):
    """**Component**: Specifies relation a spatial transform describes."""

    ParentFromChild = 1
    """
    The transform describes how to transform into the parent entity's space.

    E.g. a translation of (0, 1, 0) with this `TransformRelation` logged at `parent/child` means
    that from the point of view of `parent`, `parent/child` is translated 1 unit along `parent`'s Y axis.
    From perspective of `parent/child`, the `parent` entity is translated -1 unit along `parent/child`'s Y axis.
    """

    ChildFromParent = 2
    """
    The transform describes how to transform into the child entity's space.

    E.g. a translation of (0, 1, 0) with this `TransformRelation` logged at `parent/child` means
    that from the point of view of `parent`, `parent/child` is translated -1 unit along `parent`'s Y axis.
    From perspective of `parent/child`, the `parent` entity is translated 1 unit along `parent/child`'s Y axis.
    """

    def __str__(self) -> str:
        """Returns the variant name."""
        if self == TransformRelation.ParentFromChild:
            return "ParentFromChild"
        elif self == TransformRelation.ChildFromParent:
            return "ChildFromParent"
        else:
            raise ValueError("Unknown enum variant")


TransformRelationLike = Union[
    TransformRelation, Literal["ChildFromParent", "ParentFromChild", "childfromparent", "parentfromchild"]
]
TransformRelationArrayLike = Union[TransformRelationLike, Sequence[TransformRelationLike]]


class TransformRelationType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.TransformRelation"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union([
                pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                pa.field("ParentFromChild", pa.null(), nullable=True, metadata={}),
                pa.field("ChildFromParent", pa.null(), nullable=True, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class TransformRelationBatch(BaseBatch[TransformRelationArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = TransformRelationType()

    @staticmethod
    def _native_to_pa_array(data: TransformRelationArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (TransformRelation, int, str)):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, TransformRelation):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                if hasattr(TransformRelation, value):
                    types.append(TransformRelation[value].value)  # fast path
                elif value.lower() == "parentfromchild":
                    types.append(TransformRelation.ParentFromChild.value)
                elif value.lower() == "childfromparent":
                    types.append(TransformRelation.ChildFromParent.value)
                else:
                    raise ValueError(f"Unknown TransformRelation kind: {value}")
            else:
                raise ValueError(f"Unknown TransformRelation kind: {value}")

        buffers = [
            None,
            pa.array(types, type=pa.int8()).buffers()[1],
        ]
        children = (1 + 2) * [pa.nulls(len(data))]

        return pa.UnionArray.from_buffers(
            type=data_type,
            length=len(data),
            buffers=buffers,
            children=children,
        )
