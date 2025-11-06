"""Tests for the tilesort Python package."""

import tilesort


class TestSortBasic:
    """Basic tests for tilesort.sort() function."""

    def test_simple_two_tiles(self):
        data = [3, 4, 5, 1, 2]
        tilesort.sort(data)
        assert data == [1, 2, 3, 4, 5]

    def test_three_tiles_overlapping(self):
        data = [1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10]
        tilesort.sort(data)
        assert data == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]

    def test_already_sorted(self):
        data = [1, 2, 3, 4, 5]
        tilesort.sort(data)
        assert data == [1, 2, 3, 4, 5]

    def test_reverse_sorted(self):
        data = [5, 4, 3, 2, 1]
        tilesort.sort(data)
        assert data == [1, 2, 3, 4, 5]

    def test_single_element(self):
        data = [42]
        tilesort.sort(data)
        assert data == [42]

    def test_empty(self):
        data = []
        tilesort.sort(data)
        assert data == []

    def test_two_elements_sorted(self):
        data = [1, 2]
        tilesort.sort(data)
        assert data == [1, 2]

    def test_two_elements_unsorted(self):
        data = [2, 1]
        tilesort.sort(data)
        assert data == [1, 2]

    def test_many_tiles(self):
        data = [2, 3, 1, 4, 5, 6, 3, 7, 8, 9]
        tilesort.sort(data)
        assert data == [1, 2, 3, 3, 4, 5, 6, 7, 8, 9]

    def test_duplicates(self):
        data = [3, 3, 3, 1, 1, 2, 2]
        tilesort.sort(data)
        assert data == [1, 1, 2, 2, 3, 3, 3]

    def test_strings(self):
        data = ["cat", "dog", "elephant", "ant", "bear"]
        tilesort.sort(data)
        assert data == ["ant", "bear", "cat", "dog", "elephant"]


class TestSortReverse:
    """Tests for tilesort.sort() with reverse=True."""

    def test_reverse_simple(self):
        data = [1, 2, 3, 4, 5]
        tilesort.sort(data, reverse=True)
        assert data == [5, 4, 3, 2, 1]

    def test_reverse_two_tiles(self):
        data = [5, 4, 3, 8, 7, 6]
        tilesort.sort(data, reverse=True)
        assert data == [8, 7, 6, 5, 4, 3]

    def test_reverse_strings(self):
        data = ["ant", "bear", "cat"]
        tilesort.sort(data, reverse=True)
        assert data == ["cat", "bear", "ant"]


class TestSortedBasic:
    """Tests for tilesort.sorted() function."""

    def test_sorted_basic(self):
        data = [3, 4, 5, 1, 2]
        result = tilesort.sorted(data)
        assert result == [1, 2, 3, 4, 5]
        assert data == [3, 4, 5, 1, 2]  # Original unchanged

    def test_sorted_three_tiles(self):
        data = [1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10]
        result = tilesort.sorted(data)
        assert result == [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        assert data == [1, 2, 3, 4, 5, 11, 12, 13, 14, 15, 6, 7, 8, 9, 10]

    def test_sorted_empty(self):
        data = []
        result = tilesort.sorted(data)
        assert result == []

    def test_sorted_reverse_basic(self):
        data = [3, 4, 5, 1, 2]
        result = tilesort.sorted(data, reverse=True)
        assert result == [5, 4, 3, 2, 1]
        assert data == [3, 4, 5, 1, 2]  # Original unchanged

    def test_sorted_reverse_two_tiles(self):
        data = [5, 4, 3, 8, 7, 6]
        result = tilesort.sorted(data, reverse=True)
        assert result == [8, 7, 6, 5, 4, 3]
        assert data == [5, 4, 3, 8, 7, 6]  # Original unchanged


class TestSortWithKey:
    """Tests for tilesort.sort() with key function."""

    def test_sort_by_abs(self):
        data = [-5, -3, -1, 2, 4]
        tilesort.sort(data, key=abs)
        assert data == [-1, 2, -3, 4, -5]

    def test_sort_by_string_length(self):
        data = ["elephant", "cat", "dog", "a", "bear"]
        tilesort.sort(data, key=len)
        assert data == ["a", "cat", "dog", "bear", "elephant"]

    def test_sort_by_custom_function(self):
        data = [3, 1, 4, 1, 5, 9, 2, 6]
        tilesort.sort(data, key=lambda x: -x)  # Sort in reverse using negative key
        assert data == [9, 6, 5, 4, 3, 2, 1, 1]

    def test_sort_by_abs_reverse(self):
        data = [-5, -3, -1, 2, 4]
        tilesort.sort(data, key=abs, reverse=True)
        assert data == [-5, 4, -3, 2, -1]

    def test_sort_tuples_by_second_element(self):
        data = [(1, 5), (2, 3), (3, 1), (4, 4), (5, 2)]
        tilesort.sort(data, key=lambda x: x[1])
        assert data == [(3, 1), (5, 2), (2, 3), (4, 4), (1, 5)]


class TestSortedWithKey:
    """Tests for tilesort.sorted() with key function."""

    def test_sorted_by_abs(self):
        data = [-5, -3, -1, 2, 4]
        result = tilesort.sorted(data, key=abs)
        assert result == [-1, 2, -3, 4, -5]
        assert data == [-5, -3, -1, 2, 4]  # Original unchanged

    def test_sorted_by_string_length(self):
        data = ["elephant", "cat", "dog", "a", "bear"]
        result = tilesort.sorted(data, key=len)
        assert result == ["a", "cat", "dog", "bear", "elephant"]
        assert data == ["elephant", "cat", "dog", "a", "bear"]

    def test_sorted_by_abs_reverse(self):
        data = [-5, -3, -1, 2, 4]
        result = tilesort.sorted(data, key=abs, reverse=True)
        assert result == [-5, 4, -3, 2, -1]
        assert data == [-5, -3, -1, 2, 4]  # Original unchanged


class TestSortObjects:
    """Tests for sorting custom objects."""

    def test_sort_objects_by_attribute(self):
        class Person:
            def __init__(self, name, age):
                self.name = name
                self.age = age

            def __eq__(self, other):
                return self.name == other.name and self.age == other.age

        people = [
            Person("Charlie", 35),
            Person("Alice", 30),
            Person("Bob", 25),
            Person("Diana", 40),
            Person("Eve", 28),
        ]

        tilesort.sort(people, key=lambda p: p.age)

        assert people[0].name == "Bob"  # age 25
        assert people[1].name == "Eve"  # age 28
        assert people[2].name == "Alice"  # age 30
        assert people[3].name == "Charlie"  # age 35
        assert people[4].name == "Diana"  # age 40


class TestEdgeCases:
    """Edge case tests."""

    def test_floats(self):
        data = [3.14, 2.71, 1.41, 1.73]
        tilesort.sort(data)
        assert data == [1.41, 1.73, 2.71, 3.14]

    def test_mixed_positive_negative(self):
        data = [3, -1, 4, -5, 2, -3]
        tilesort.sort(data)
        assert data == [-5, -3, -1, 2, 3, 4]

    def test_large_duplicates(self):
        data = [5] * 10 + [3] * 10 + [7] * 10 + [1] * 10
        tilesort.sort(data)
        assert data == [1] * 10 + [3] * 10 + [5] * 10 + [7] * 10

    def test_none_values_with_key(self):
        # Test that we can handle None values with appropriate key function
        data = [3, None, 1, None, 2]
        tilesort.sort(data, key=lambda x: float("inf") if x is None else x)
        assert data == [1, 2, 3, None, None]
