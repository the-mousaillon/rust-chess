import * as React from "react"
import { ChessBoard } from "../components/chessboard"
import { Piece } from "../components/piece"

export default function ChessDev() {
    let initial_board = []
    for (let i=0; i<8; i++) {
        let row = []
        for (let j=0; j<8; j++) {
            row.push({
                piece: {
                    idx: 6000,
                }
            })
        }
        initial_board.push(row)
    }
    let [board, set_board] = React.useState(initial_board)
    console.log("initial board: ", initial_board)
    return (
        <div style={{
            display: "flex"
        }}>
        <input type="text" style={{
            height: "500px",
            width: "800px",
        }}
        placeholder="input the repr here"
        onChange={
            (ev) => {
                console.log(ev)
                try {
                    let board = JSON.parse(ev.target.value)
                    console.log("got bord: ", board)
                    set_board(board.board)
                }
                catch {

                }
            }
        }
        >

        </input>
        <ChessBoard board={board}/>
        </div>
    )
}