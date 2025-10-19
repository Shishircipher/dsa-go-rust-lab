// This is better approach of the problem.
package main

import (
	"fmt"
)

func main() {
     
	arr := [][]int {
		{1, 1, 1},
		{1,0,1},
		{1,1,1},
	}
    
	setMatrixZero(arr)

	fmt.Println("Result:")
    // https://go.dev/tour/moretypes/16 https://go.dev/tour/concurrency/4 https://go.dev/wiki/RangefuncExperiment
	// https://boldlygo.tech/posts/2024-07-18-range-over-func/
	for _, row := range arr {
		fmt.Println(row)
	}


}

func setMatrixZero(matrix [][]int) {
	// https://go.dev/tour/moretypes/15
	// rows, cols := []int{}, []int{}
	var rows, cols []int

	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			if matrix[i][j] == 0 {
				rows = append(rows, i)
				cols = append(cols, j)
			}
		}
	}

	for i := 0; i < len(rows); i++ {
		for j := 0; j < len(matrix[0]); j++ {
			r := rows[i]
			matrix[r][j] = 0
		}
	}

	for i := 0; i < len(cols); i++ {
		for j := 0; j < len(matrix); j++ {
			c := cols[i]
			matrix[j][r] = 0
		}
	}
}

// In next version, Use maps to store unique indices: - To avoid duplicate row/col zeroing - 
// If the matrix has multiple zeros, rows or cols can contain duplicates.

