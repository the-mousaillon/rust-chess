import * as React from "react"
import './board.css';
import { Cell } from './cell';
import Switch from '@mui/material/Switch';
import { Promotion } from "./promotion";

interface Piece {
    idx: number,
    threatened?: boolean,
    color?: "WHITE" | "BLACK"
}

interface Board{
    board: Piece[][]
}


const key_listener = (set_board: any) => (ev: any) => {
    let url;
    if (ev.key == "ArrowLeft") {
        url = "http://localhost:8005/api/get_previous_board"
    }
    else if (ev.key == "ArrowRight") {
        url = "http://localhost:8005/api/get_next_board"
    }
    else {
        url = ""
    }
    if (url != "") {
        fetch(url, {
            method: "GET",
            mode: "cors"
        }).then((r) => {
            r.json().then(b => {
                console.log("history resp: ", b)
                set_board(b.board)
            })
            .catch(e => console.log("err: ", e))
        })
        .catch(e => console.log("err: ", e))
    }
}

export const ChessBoard = (props: {
    board: Board,
    remote_engine: boolean,
    set_board?: any
}) => {
    let boardJSX = [];
    console.log("Board in chessboard: ", props.board)
    let [show_controll, set_show_controll] = React.useState(false)
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            let color = (i + j) % 2 == 0? "white": "black"
            boardJSX.push(
                <Cell 
                    key={i * 8 + j}
                    color={color}
                    piece={props.board[i][j].piece}
                    threatened={props.board[i][j].threatened}
                    controll={props.board[i][j].controll}
                    remote_engine={props.remote_engine}
                    x={i}
                    y={j}
                    set_board={props.set_board}
                    show_controll={show_controll}
                />
            )
        }
    }
    let listener = key_listener(props.set_board)
    React.useEffect(() => {
        window.addEventListener("keydown", listener)
    }, [])
    // ArrowLeft, ArrowRight, ev.key
    return (
        <div>
            <Switch
                checked={show_controll}
                onChange={() => set_show_controll(!show_controll)}
                name="loading"
            />
            <div style={{
                display: "flex",
                flexDirection: "column"
            }}>
                <Promotion color="WHITE" set_board={props.set_board}/>
                <div className="board">
                    { boardJSX }
                </div>
                <Promotion color="BLACK" set_board={props.set_board}/>
            </div>
        </div>
    )
}