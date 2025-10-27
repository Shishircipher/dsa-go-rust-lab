Problem Statement: Given the root of a Binary Tree (Level-Order (Breadth-First) representation of a Binary Tree), write a recursive function that returns an array containing the inorder traversal of the tree.

Example 1:
Input: Binary Tree: 4 2 5 3 -1 7 6 -1 9 -1 -1 8 -1 1

                4
           /         \
         2             5
       /   \         /   \
      3    -1       7      6
     / \          /     /
    -1   9        8     1


Output: Output: [3 9 2 4 8 7 5 1 6]
Explanation: We traverse the binary tree in the order of Left, Root then Right recursively resulting in the following traversal: