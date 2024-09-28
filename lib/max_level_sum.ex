# 1161. Maximum Level Sum of a Binary Tree
# https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree

defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil
end

defmodule Solution do
  @spec max_level_sum(root :: TreeNode.t() | nil) :: integer
  def max_level_sum(root) do
    root
    |> node_sums()
    |> Enum.with_index()
    |> Enum.max(fn {lhs, _}, {rhs, _} -> lhs <= rhs end)
    |> Kernel.elem(1)
  end

  def node_sums(nil), do: []

  def node_sums(node) do
    left = node_sums(node.left)
    right = node_sums(node.right)
    [node.val | sum_list(left, right)]
  end

  def sum_list([], rhs), do: rhs
  def sum_list(lhs, []), do: lhs

  def sum_list([lhs | lhs_tail], [rhs | rhs_tail]) do
    [lhs + rhs | sum_list(lhs_tail, rhs_tail)]
  end
end
