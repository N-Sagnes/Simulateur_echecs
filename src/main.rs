fn cases_possibles (piece:i32, posi:i32, posj:i32, &board:[[i32; 8]; 8]) 
-> Vec<((i32,i32))> {
    let mut cases_p = vec![]; 
    let joueur = -(piece%2*2-1);
    //pion
    if piece>=17 {
        // devant 
        if posi<=6{
        if board[posi+joueur][posj] == 0 {
            cases_p.push((posi+joueur, posj));
            //devant *2
            if 7 - joueur*5 - 2*posi == 0 {
                if board[posi+2*joueur][posj] == 0 {
                    cases_p.push((posi+2*joueur, posj))
                }
            }
        }
        //manger
        if posj>=1{
            if (board[posi+joueur][posj-1]>0) & ((board[posi+joueur][posj-1]+piece)%2 ==1) {
                cases_p.push((posi+joueur,posj-1))
            };
        }
        if posj<=6{
            if (board[posi+joueur][posj+1]>0) & ((board[posi+joueur][posj+1]+piece)%2 ==1) {
                cases_p.push((posi+joueur,posj+1))
            };
        }
        }; 
    }
cases_p
}

fn main() {
    let board = [
    [6, 10, 14, 4, 2, 16, 12, 8],
    [18, 20, 22, 24, 26, 28, 30, 32],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [17, 19, 21, 23, 25, 27, 29, 31],
    [5, 9, 13, 3, 1, 15, 11, 7],
]; 
cases_possibles(18,1,0,board)
}
