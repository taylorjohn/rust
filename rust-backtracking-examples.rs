// 1. N-Queens Problem
fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    let mut solutions = Vec::new();
    let mut board = vec![vec!['.'; n]; n];
    
    fn backtrack(row: usize, board: &mut Vec<Vec<char>>, solutions: &mut Vec<Vec<String>>) {
        if row == board.len() {
            solutions.push(board.iter().map(|row| row.iter().collect()).collect());
            return;
        }
        
        for col in 0..board.len() {
            if is_safe(board, row, col) {
                board[row][col] = 'Q';
                backtrack(row + 1, board, solutions);
                board[row][col] = '.';
            }
        }
    }
    
    fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
        // Check column
        for i in 0..row {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        
        // Check upper left diagonal
        let (mut i, mut j) = (row as i32 - 1, col as i32 - 1);
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        
        // Check upper right diagonal
        let (mut i, mut j) = (row as i32 - 1, col as i32 + 1);
        while i >= 0 && j < board.len() as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        
        true
    }
    
    backtrack(0, &mut board, &mut solutions);
    solutions
}

// 2. Sudoku Solver
fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, num: char) -> bool {
        // Check row
        if board[row].contains(&num) {
            return false;
        }
        
        // Check column
        if board.iter().any(|r| r[col] == num) {
            return false;
        }
        
        // Check 3x3 box
        let box_row = row - row % 3;
        let box_col = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if board[box_row + i][box_col + j] == num {
                    return false;
                }
            }
        }
        
        true
    }
    
    fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    for num in '1'..='9' {
                        if is_valid(board, row, col, num) {
                            board[row][col] = num;
                            if backtrack(board) {
                                return true;
                            }
                            board[row][col] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        true
    }
    
    backtrack(board)
}

// 3. Subset Sum Problem
fn subset_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    
    fn backtrack(nums: &[i32], target: i32, start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(current.clone());
            return;
        }
        
        for i in start..nums.len() {
            if nums[i] > target {
                continue;
            }
            current.push(nums[i]);
            backtrack(nums, target - nums[i], i + 1, current, result);
            current.pop();
        }
    }
    
    backtrack(nums, target, 0, &mut current, &mut result);
    result
}

// 4. Permutations
fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    
    fn backtrack(nums: &[i32], used: &mut Vec<bool>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            current.push(nums[i]);
            backtrack(nums, used, current, result);
            current.pop();
            used[i] = false;
        }
    }
    
    backtrack(nums, &mut used, &mut current, &mut result);
    result
}

// 5. Word Search
fn exist(board: &Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    let m = board.len();
    let n = board[0].len();
    let mut visited = vec![vec![false; n]; m];
    
    fn dfs(board: &Vec<Vec<char>>, word: &[char], i: i32, j: i32, k: usize, visited: &mut Vec<Vec<bool>>) -> bool {
        if k == word.len() {
            return true;
        }
        
        if i < 0 || i >= board.len() as i32 || j < 0 || j >= board[0].len() as i32 
            || visited[i as usize][j as usize] || board[i as usize][j as usize] != word[k] {
            return false;
        }
        
        visited[i as usize][j as usize] = true;
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (di, dj) in directions.iter() {
            if dfs(board, word, i + di, j + dj, k + 1, visited) {
                return true;
            }
        }
        
        visited[i as usize][j as usize] = false;
        false
    }
    
    for i in 0..m {
        for j in 0..n {
            if dfs(board, &word, i as i32, j as i32, 0, &mut visited) {
                return true;
            }
        }
    }
    
    false
}

fn main() {
    // 1. N-Queens
    println!("N-Queens (4x4):");
    let solutions = solve_n_queens(4);
    for (i, solution) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        for row in solution {
            println!("{}", row);
        }
        println!();
    }

    // 2. Sudoku Solver
    let mut sudoku_board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    println!("Sudoku Solver:");
    if solve_sudoku(&mut sudoku_board) {
        for row in &sudoku_board {
            println!("{:?}", row);
        }
    } else {
        println!("No solution exists");
    }

    // 3. Subset Sum
    let nums = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    println!("\nSubset Sum (target = {}):", target);
    let subsets = subset_sum(&nums, target);
    for subset in subsets {
        println!("{:?}", subset);
    }

    // 4. Permutations
    let nums = vec![1, 2, 3];
    println!("\nPermutations of {:?}:", nums);
    let perms = permutations(&nums);
    for perm in perms {
        println!("{:?}", perm);
    }

    // 5. Word Search
    let board = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    let word = "ABCCED".to_string();
    println!("\nWord Search for '{}': {}", word, exist(&board, word));
}
