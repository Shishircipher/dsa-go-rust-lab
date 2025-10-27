package main

import (
	"fmt"
)

type binaryTree []int

func main() {
   
	//binTree := []int{4, 2, 5, 3, -1, 7, 6, -1, 9, -1, -1, 8, -1, 1}
	// binTree := binaryTree{4, 2, 5, 3, -1, 7, 6, -1, 9, -1, -1, 8, -1, 1}
	// Level-Order (Breadth-First) representation of a Binary Tree
	binTree := binaryTree{1, 2, 3, 4, 5, 6, 7, -1,-1, 8,-1,-1,-1, 9,10}
	fmt.Println("Inorder:", inorderTraversal(binTree))

}




func inorderTraversal(v binaryTree) []int {
    // i := 0
	var result []int
	n := len(v)
	var traverse func(v binaryTree, i, n int)
	traverse = func(v binaryTree, i, n int) {
		if i >= n || v[i] == -1 {
			return
		}
         left := 2*i + 1
		 right := 2*i + 2
		 if (left < n && v[left] != -1 ){
            traverse(v, left, n)
		 } 

		 result = append(result, v[i])

		 if (right < n && v[right] != -1 ) {
			traverse(v, right, n)
		 } 
		
		 
		
	}

	traverse(v, 0, n)

	return result


}

func (v binaryTree) leftNode(i int) int {
	 
	return v[(2*i + 1)]
}

func (v binaryTree) rightNode(i int) int {
	return v[(2*i + 2)]
}