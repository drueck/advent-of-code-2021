# Advent of Code 2021: Day 8, Part 2
# https://adventofcode.com/2021/day/8
# Usage: elixir main.exs <input-file>

# Algorithm Notes
#
# Start by mapping the segments associated with the digits with unique segment lengths
#
# 2 segments -> 1
# 4 segments -> 4
# 3 segments -> 7
# 7 segments -> 8
#
# Then use that knowledge to try to disambiguate the digits that share the same segment
# length by calculations based on commonalities or differences between those digits' segments
# and the known digits' segments.
#
# Note: The order matters in these comparisons below.
#
# length 5 ->
#     contains all segments from 1                                 -> 3
#     contains segments from the difference between 4 - 1)         -> 5
#     does not contain segments from the difference between 4 - 1) -> 2
#
# 6 segments ->
#     does not contain all segments from 1                         -> 6
#     contains all segments from 4                                 -> 9
#     does not contain all segments from 4                         -> 0

defmodule DayEight do
  @known_segments_digits %{2 => 1, 4 => 4, 3 => 7, 7 => 8}
  @known_segments_lengths MapSet.new(Map.keys(@known_segments_digits))

  def segments_string_to_set(segments) do
    segments
    |> String.codepoints()
    |> MapSet.new()
  end

  def map_digits(all_digits) do
    {known, unknown} =
      all_digits
      |> String.split(" ")
      |> Enum.map(&segments_string_to_set/1)
      |> Enum.map(fn digit_segments ->
        {digit_segments, MapSet.size(digit_segments)}
      end)
      |> Enum.split_with(fn {_, size} -> size in @known_segments_lengths end)

    digits_map =
      known
      |> Enum.reduce(%{}, fn
        {segments, 2}, digits_map -> Map.put(digits_map, 1, segments)
        {segments, 4}, digits_map -> Map.put(digits_map, 4, segments)
        {segments, 3}, digits_map -> Map.put(digits_map, 7, segments)
        {segments, 7}, digits_map -> Map.put(digits_map, 8, segments)
      end)

    unknown
    |> Enum.reduce(digits_map, fn
      {segments, 5}, digits_map ->
        four_minus_one = MapSet.difference(digits_map[4], digits_map[1])

        cond do
          MapSet.subset?(digits_map[1], segments) -> Map.put(digits_map, 3, segments)
          MapSet.subset?(four_minus_one, segments) -> Map.put(digits_map, 5, segments)
          !MapSet.subset?(four_minus_one, segments) -> Map.put(digits_map, 2, segments)
          true -> raise("Failed to match digit with 5 segments")
        end

      {segments, 6}, digits_map ->
        cond do
          !MapSet.subset?(digits_map[1], segments) -> Map.put(digits_map, 6, segments)
          MapSet.subset?(digits_map[4], segments) -> Map.put(digits_map, 9, segments)
          !MapSet.subset?(digits_map[4], segments) -> Map.put(digits_map, 0, segments)
          true -> raise("Failed to match digit with 6 segments")
        end
    end)
    |> Map.new(fn {digit, segments} -> {segments, digit} end)
  end

  def run do
    System.argv()
    |> Enum.at(0)
    |> File.stream!()
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.split(&1, " | "))
    |> Stream.map(fn [all_digits, code_digits] ->
      digits_for_segments = map_digits(all_digits)

      code_digits
      |> String.split(" ")
      |> Stream.map(&segments_string_to_set/1)
      |> Stream.map(&Map.get(digits_for_segments, &1))
      |> Enum.join()
      |> String.to_integer()
    end)
    |> Enum.sum()
    |> IO.inspect()
  end
end

DayEight.run()
