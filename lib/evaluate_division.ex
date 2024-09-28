defmodule EvaluateDivision do
  @moduledoc """
  https://leetcode.com/problems/evaluate-division/
  """

  @type optional_value :: float() | nil

  defp dps(graph, start_node, end_node) do
    dps(graph, start_node, end_node, MapSet.new() |> MapSet.put(start_node))
  end

  defp dps_loop(graph, {next_node, weight}, end_node, visited) do
    if next_node in visited do
      nil
    else
      if result = dps(graph, next_node, end_node, MapSet.put(visited, next_node)) do
        result * weight
      else
        nil
      end
    end
  end

  defp dps_conn(graph, connections, end_node, visited) do
    if result = Map.get(connections, end_node) do
      result
    else
      Enum.find_value(connections, fn item -> dps_loop(graph, item, end_node, visited) end)
    end
  end

  defp dps(graph, start_node, end_node, visited) do
    dps_conn(graph, Map.get(graph, start_node), end_node, visited)
  end

  @spec find_weight(%{String.t() => %{String.t() => float}}, String.t(), String.t()) ::
          optional_value()
  defp find_weight(graph, start_node, end_node) do
    cond do
      not Map.has_key?(graph, start_node) or not Map.has_key?(graph, end_node) -> nil
      start_node == end_node -> 1
      true -> dps(graph, start_node, end_node)
    end
  end

  @spec unwrap_optional(optional_value()) :: float()
  defp unwrap_optional(nil), do: -1
  defp unwrap_optional(value), do: value

  @spec calc_equation(equations :: [[String.t()]], values :: [float], queries :: [[String.t()]]) ::
          [float]
  def calc_equation(equations, values, queries) do
    graph =
      for {[top_var, bottom_var], value} <- Enum.zip(equations, values), reduce: %{} do
        acc ->
          acc
          |> Map.update(top_var, %{bottom_var => value}, fn connections ->
            Map.put(connections, bottom_var, value)
          end)
          |> Map.update(bottom_var, %{top_var => 1 / value}, fn connections ->
            Map.put(connections, top_var, 1 / value)
          end)
      end

    for [top_var, bottom_var] <- queries do
      find_weight(graph, top_var, bottom_var) |> unwrap_optional()
    end
  end
end
