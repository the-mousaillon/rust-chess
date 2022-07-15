import * as React from "react"
import './board.css';
import { Cell } from './cell';

interface Piece {
    idx: number,
    threatened?: boolean,
    color?: "WHITE" | "BLACK"
}

interface Board{
    board: Piece[][]
}

export const ChessBoard = (props: {
    board: Board
}) => {
    let boardJSX = [];
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            let color = (i + j) % 2 == 0? "white": "black"
            boardJSX.push(
                <Cell key={i * 8 + j} color={color} piece={props.board[i][j].piece}/>
            )
        }
    }
    return (
        <div className="board">
            { boardJSX }
        </div>
    )
}