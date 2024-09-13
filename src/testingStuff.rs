//Early version, still testing stuff, etc. etc.

#![allow(warnings)]

use std::num;


struct Game {
    turn_counter: usize,
    board: Board,
    last_capture: usize
}
struct Board {
    positions: [[Position; 8]; 8],
    //needed?
    //pieces_left: Vec<Piece>
}
#[derive(Copy, Clone)]
struct Position {
    is_empty : bool,
    content : PositionContent,
    x: usize,
    y: usize
}

#[derive(Copy, Clone)]
enum PositionContent {
    PIECE_CONT(Piece),
    NONE
}

impl PositionContent {
    fn print(self)-> char{
        match self {
            PositionContent::NONE => return 'L',
            PositionContent::PIECE_CONT(mut p) => return p.print()
        }
    }
}

#[derive(Copy, Clone)]
struct Piece {
    variant : PieceType,
    color : Color,
    has_moved : bool
}

impl Piece {
    fn print(&mut self) -> char {
        let mut ret_char: char = 'N';
        match self.variant {
            PieceType::PAWN => ret_char = if matches!(self.color, Color::W) {'♟' } else {'♙' },
            PieceType::KNIGHT => ret_char = if matches!(self.color, Color::W) { '♞'} else {'♘' }, 
            PieceType::BISHIOP => ret_char = if matches!(self.color, Color::W) {'♝' } else {'♗' },
            PieceType::ROOK => ret_char = if matches!(self.color, Color::W) {'♜' } else {'♖' },
            PieceType::QUEEN => ret_char = if matches!(self.color, Color::W) { '♛'} else {'♕'},
            PieceType::KING => ret_char = if matches!(self.color, Color::W) { '♚'} else { '♔'},
            _ => ret_char = 'L'
        }
        return ret_char;
    }
    //provides the longest possible path that each piece can take in an ideal scenario
    fn get_paths(self) -> Vec<(i16,i16)> {
        match self.variant {
            PieceType::PAWN => vec![(0,1)],
            PieceType::KNIGHT => vec![(1,2),(2,1),(-1,2),(2,-1),(-1,-2),(-2,-1),(1,-2),(-2,1)], 
            PieceType::BISHIOP => vec![(-7,7),(7,7),(-7,-7),(7,-7)],
            PieceType::ROOK => vec![(0,7),(-7,0),(7,0),(0,-7)],
            PieceType::QUEEN => vec![(-7,7),(0,7),(7,7),(-7,0),(7,0),(-7,-7),(0,-7),(7,-7)],
            PieceType::KING => vec![(-1,1),(0,1),(1,1),(-1,0),(1,0),(-1,-1),(0,-1),(1,-1)],
            _ => return vec![]
        }
    }
}
#[derive(Copy, Clone)]
enum PieceType {
    PAWN,
    KNIGHT,
    BISHIOP,
    ROOK,
    QUEEN,
    KING, 
    NONE
}
#[derive(Copy, Clone)]
enum Color {
    W,
    B
}

fn convert_notation(chess_not: &str)-> (usize,usize){
    let mut chars = chess_not.chars();
    let x_coord:char = chars.next().unwrap();
    let y_coord:char = chars.next().unwrap();

    let mut x_converted:usize = 0;
    let mut y_converted:usize = 0;

    match x_coord {
        'a'| 'A' => x_converted = 0,
        'b'| 'B' => x_converted = 1,
        'c'| 'C' => x_converted = 2,
        'd'| 'D' => x_converted = 3,
        'e'| 'E' => x_converted = 4,
        'f'| 'F' => x_converted = 5,
        'g'| 'G' => x_converted = 6,
        'h'| 'H' => x_converted = 7,
        _ => panic!()
    };

    y_converted = y_coord.to_digit(32).unwrap() as usize - 1;


    return (x_converted,y_converted);
}

impl Game {
    fn start(&mut self) {
        self.board.reset();
        self.turn_counter = 1;
        self.last_capture = 0;
    }

    fn move_piece(&mut self, from: &str, to: &str) {
        let f_coords = convert_notation(from);
        let t_coords = convert_notation(to);

        self.turn_counter += 1;
    }

    fn get_legal_moves(&mut self, from: &str) -> Vec<&str> {
        let f_coords: (usize, usize) = convert_notation(from);
        let f_content: PositionContent = self.board.positions[f_coords.0][f_coords.1].content;

        match f_content {
            PositionContent::NONE => return vec![],
            PositionContent::PIECE_CONT(mut p) => {
                if matches!(p.color,Color::W) && self.turn_counter % 2 == 1 {
                    return vec![]  
                } else if matches!(p.color,Color::B) && self.turn_counter % 2 == 0 {
                    return vec![] 
                } else {
                    let mut paths: Vec<(i16, i16)> = p.get_paths();
                    match p.variant {
                        //TODO
                        //PieceType::KNIGHT|PieceType::BISHIOP|PieceType::ROOK|PieceType::QUEEN => {},
                        //PieceType::KING => {},
                        //PieceType::PAWN => {/*should not use paths, custom logic must be implemented*/}
                        _ => panic!()
                    } 
                }
            }
        }
    }
    //TODO
    fn check_victory(&mut self, color:Color) -> bool {
        return false;
    }

}

impl Board {
    //add back pieces to the vec
    fn reset(&mut self) {
        
        for i in 0..8 {
            for j in 0..8 {
                match j {
                    //black figures
                    0 => {
                        let mut figure = PieceType::NONE;

                        match i {
                            0 => figure = PieceType::ROOK,
                            1 => figure = PieceType::KNIGHT,
                            2 => figure = PieceType::BISHIOP,
                            3 => figure = PieceType::QUEEN,
                            4 => figure = PieceType::KING,
                            5 => figure = PieceType::BISHIOP,
                            6 => figure = PieceType::KNIGHT,
                            7 => figure = PieceType::ROOK,
                            _ => panic!()
                        };

                        self.positions[i][j] = Position {
                            is_empty : false,
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : figure,
                                color : Color::B,
                                has_moved: false
                            }),
                            x : j,
                            y : i 
                        };

                    },
                    //pawns black
                    1 => {
                        self.positions[i][j] = Position {
                            is_empty : false,
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : PieceType::PAWN,
                                color : Color::B,
                                has_moved: false
                            }),
                            x : j,
                            y : i 
                        };
                    },
                    
                    //pawns white
                    6 => {
                        self.positions[i][j] = Position {
                            is_empty : false,
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : PieceType::PAWN,
                                color : Color::W,
                                has_moved: false
                            }),
                            x : j,
                            y : i 
                        };
                    },
                    
                    //white figures
                    7 => {  
                        let mut figure = PieceType::NONE;

                        match i {
                            0 => figure = PieceType::ROOK,
                            1 => figure = PieceType::KNIGHT,
                            2 => figure = PieceType::BISHIOP,
                            3 => figure = PieceType::QUEEN,
                            4 => figure = PieceType::KING,
                            5 => figure = PieceType::BISHIOP,
                            6 => figure = PieceType::KNIGHT,
                            7 => figure = PieceType::ROOK,
                            _ => panic!()
                        };

                        self.positions[i][j] = Position {
                            is_empty : false,
                            content : PositionContent::PIECE_CONT(Piece { 
                                variant : figure,
                                color : Color::W,
                                has_moved: false
                            }),
                            x : j,
                            y : i 
                        };
                    },

                    _ => {
                        self.positions[i][j] = Position {
                            is_empty : true,
                            content : PositionContent::NONE,
                            x : j,
                            y : i 
                        };
                    }
                }
                
            }
        }
        
      
    }
    fn print_state(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                let c_piece = self.positions[j][i].content.print();
                print!("{c_piece} ");
            }
            println!("");
        }
    }
    //TODO
    fn is_in_check(self,king:Position)->bool{
        return false;
    }
    
    fn force_move(&mut self,from:(usize,usize),to:(usize,usize)){
        self.positions[to.0][to.1] = self.positions[from.0][from.1]; 
        self.positions[from.0][from.1] = Position{is_empty:true,content:PositionContent::NONE,x:from.0,y:from.1};
    }

    fn traverse_path(self, from:Position,path:(i16,i16)) -> Vec<(usize,usize)>{
        //establishes direction of movement along the path
        let x_dir: i16 = if path.0 < 0 {-1 as i16} else if path.0 > 0 {1 as i16} else {0 as i16};
        let y_dir: i16 = if path.1 < 0 {-1 as i16} else if path.1 > 0 {1 as i16} else {0 as i16}; 

        //temp value
        let mut p_col = Color::W;

        //list of all viable moves
        let mut possible_moves:Vec<(usize,usize)> = Vec::new(); 

        match from.content {
            PositionContent::NONE => panic!(),
            PositionContent::PIECE_CONT(mut p)=> { 
                p_col = p.color;
            }
        }

        let mut curr_x =x_dir;
        let mut curr_y =y_dir;


        while curr_x.abs() <= x_dir.abs() && curr_y.abs() <= y_dir.abs(){
            //out of bounds on x
            if (from.x as i16 + curr_x<0) || (from.x as i16 + curr_x>7) {
                break;
            }
            //out of bounds on y
            if (from.y as i16 +curr_y<0) || (from.y as i16 +curr_y>7) {
                break;
            }
            //still in bounds, check if the spot is occupied by ally/enemy piece
            let mut eval_pos = self.positions[(from.x as i16 + curr_x) as usize][(from.y as i16 + curr_y) as usize];

            match eval_pos.content {
                PositionContent::NONE => {possible_moves.push(( (from.x as i16 + curr_x) as usize ,(from.y as i16 + curr_y) as usize  ));},
                PositionContent::PIECE_CONT(nxt_p) => {
                    //Done this way because of the limiatation of matches! function
                    if (matches!(p_col, Color::B) && matches!(nxt_p.color, Color::B)) || (matches!(p_col, Color::W) && matches!(nxt_p.color, Color::W)) {
                        //space is occupied by a piece of the same 'team'
                        break;
                    } else {
                        //enemy piece, can take but must stop afterwards
                        possible_moves.push(( (from.x as i16 + curr_x) as usize ,(from.y as i16 + curr_y) as usize  ));
                        break;
                    }
                }
            }


            //move to the next spot
            curr_y += y_dir;
            curr_x += x_dir;
        }
        return possible_moves;
    }

}
//  A   B   C   D   E   F   G   H
//8 RB  NB  BB  QB  KB  BB  NB  RB
//7 PB  PB  PB  PB  PB  PB  PB  PB
//6
//5
//4
//3
//2 PW  PW  PW  PW  PW  PW  PW  PW  
//1 RW  NW  BW  QW  KW  BW  NW  RW

fn main(){
    

    let mut game_board: Board = Board{positions: [[Position{is_empty:true, content:PositionContent::NONE,x:0,y:0}; 8]; 8], };
    game_board.reset();
    game_board.print_state();

    let tst = "a4";

    let tup_tst = convert_notation(tst);

    println!("{:?}", tup_tst);
}