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


export const ChessBoard = (props: {
    board?: Board,
    remote_engine: boolean,
    set_game_state?: any
}) => {
    let boardJSX = [];
    let [show_controll, set_show_controll] = React.useState(false)
    if (props.board) {
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
                        set_game_state={props.set_game_state}
                        show_controll={show_controll}
                    />
                )
            }
        }
    }
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
                <Promotion color="WHITE" set_game_state={props.set_game_state}/>
                <div className="board">
                    { boardJSX }
                </div>
                <Promotion color="BLACK" set_game_state={props.set_game_state}/>
            </div>
        </div>
    )
}