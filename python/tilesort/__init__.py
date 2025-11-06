"""
Tilesort - A sorting algorithm optimized for datasets with pre-sorted contiguous blocks.

This module provides sorting functions similar to Python's built-in sort() and sorted(),
but optimized for data that consists of pre-sorted tiles.
"""

from tilesort._tilesort import *  # noqa: F403

__all__ = ["sort", "sorted"]  # noqa: F405
