# Chess Library

This library provides functionality to play and interact with a chess game programmatically.

## Methods


  **start() -> Game** 

  
   Returns the Game object and sets up the game, the rest of the methods operate on the Game object 


  **get_board_representation() -> String**

  Returns a String that represents the current chessboard state. Rows are separated by /, white pieces are represented by uppercase letters, and black pieces are represented by lowercase letters.

  **check_state() -> GameState**

  Returns the current game state as an enum (ONGOING, WIN_W, WIN_B, or DRAW). If the state is not ONGOING, the game has concluded, and a new game must be started or restarted.

  **restart()**

  Resets the game to the initial board configuration, allowing for a fresh start without needing to create a new Game object.

  **move_piece(from: &str, to: &str, promotion: &str) -> bool**

  Attempts to move a piece from the 'from' position to the 'to' position using standard chess notation (e.g., "a2" to "a3"). The promotion argument is required when a pawn is being promoted and can take the following values: "Q" (Queen), "N" (Knight), "B" (Bishop), "R" (Rook), or "X" (placeholder for regular moves). Returns true if the move is valid and was made successfully.

  
  **get_all_legal_moves() -> Vec<(String, String)>**

  Returns a vector of all possible legal moves in the format of (from, to), where both from and to are in standard chess notation.

  **get_legal_moves(from: &str) -> Vec<(usize, usize)>**

  Returns all legal moves for the piece at the given position in (usize, usize) format. You can use encode_notation to convert these coordinates to standard chess notation.

  **encode_notation(pos: (usize, usize)) -> String**

  Transforms a tuple of (usize, usize) coordinates into a chess notation string (e.g., (0, 0) becomes "a1", (7,7) becomes "h8").

  **decode_notation(chess_not: &str) -> (usize, usize)**

  Reverses the transformation from chess notation into a tuple of (usize, usize) coordinates.
  
