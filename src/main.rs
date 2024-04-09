use std::usize;

// ゲームの状態を表す構造体
struct GameState {
    board: [[Option<Player>; 5]; 5], // ゲームボード
    current_player: Player,          // 現在のプレイヤー
}

impl GameState {
    // fn move_cell(player_move: PlayerMove,game_state:GameState) -> {
    // match player_move.direction{
    //     Direction::Right => move_cell_right(player_move.cell,game_state),
    //     ()=>(),
    // }

    pub fn run(&mut self, player_move: PlayerMove) {
        // TODO: 入力値のチェック
        match player_move.direction {
            Direction::Up => self.move_cell_up(player_move.cell),
            Direction::Down => self.move_cell_down(player_move.cell),
            Direction::Left => self.move_cell_left(player_move.cell),
            Direction::Right => self.move_cell_right(player_move.cell),
        }

        self.switch_player();
    }

    fn move_cell_up(&mut self, cell: Position) {
        // i: 次の盤面でのy座標
        for i in (0..5).rev() {
            if i == 0 {
                self.board[i][cell.x] = Some(self.current_player);
            } else {
                self.board[i][cell.x] = self.board[i - 1][cell.x];
            }
        }
    }

    fn move_cell_down(&mut self, cell: Position) {
        // i: 次の盤面でのy座標
        for i in 0..5 {
            if i == 4 {
                self.board[i][cell.x] = Some(self.current_player);
            } else {
                self.board[i][cell.x] = self.board[i + 1][cell.x];
            }
        }
    }

    fn move_cell_right(&mut self, cell: Position) {
        // i: 次の盤面でのx座標
        for i in (0..5).rev() {
            if i == 0 {
                self.board[cell.y][i] = Some(self.current_player);
            } else {
                self.board[cell.y][i] = self.board[cell.y][i - 1];
            }
        }
    }

    fn move_cell_left(&mut self, cell: Position) {
        // i: 次の盤面でのx座標
        for i in 0..5 {
            if i == 4 {
                self.board[cell.y][i] = Some(self.current_player);
            } else {
                self.board[cell.y][i] = self.board[cell.y][i + 1];
            }
        }
    }

    // ユーザを切り替える
    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
        };
    }
}

// プレイヤーを表すenum
#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    Circle,
    Cross,
}

// ゲームの実装
impl GameState {
    // 新しいゲームの開始
    fn new() -> Self {
        let mut board = [[None; 5]; 5];

        GameState {
            board,
            current_player: Player::Circle,
        }
    }

    // ゲームの状態を表示する
    fn display(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(Player::Circle) => print!("◯ "),
                    Some(Player::Cross) => print!("× "),
                    None => print!(". "),
                }
            }
            println!();
        }
    }

    // ゲームの終了条件をチェックする（例：全てのマスが埋まっているかどうか）
    fn is_game_over(&self) -> bool {
        // ここに終了条件のチェックを実装する
        false
    }

    // ゲームの結果を表示する（例：各プレイヤーの石の数を数えて勝者を決定する）
    fn display_result(&self) {
        // ここに結果の表示を実装する
    }
}

struct PlayerMove {
    cell: Position,
    direction: Direction,
}

struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

// ゲームのプレイ
fn main() {
    let mut game = GameState::new();
    println!("Current Board:");
    game.display();
    for _ in 0..4 {
        game.run(PlayerMove {
            cell: Position { x: 0, y: 0 },
            direction: Direction::Right,
        });
        println!("Current Board:");
        game.display();
    }

    // ゲームの結果を表示
    game.display_result();
}
