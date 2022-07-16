import * as React from "react"
import { ChessBoard } from "../components/chessboard"
import { Piece } from "../components/piece"

export default function ChessRust() {
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
    React.useEffect(() => {
            try {
            fetch("http://localhost:8005/api/reset_board", {
                method: "GET",
                mode: "cors"
            })
            .then(
                v => v.json()
                      .then(d => {
                        console.log("Got board: ", d)
                        set_board(d.board)
                      })
                      .catch(e => {console.log(e)})
            )
            .catch(e => {console.log(e)})
        }
        catch {

        }
    }, [])
    console.log("initial board: ", initial_board)
    return (
        <ChessBoard board={board} remote_engine={true} set_board={set_board} />
    )
}