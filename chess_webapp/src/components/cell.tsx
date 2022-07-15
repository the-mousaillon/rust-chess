import * as React from 'react';
import Piece from './piece';
import './cell.css'

export const Cell = (props: {
    color: string,
    piece?: {
        idx?: number,
        name?: string
        color?: "WHITE" | "BLACK"
    }
}) => {
    console.log("piece props: ", props.piece)
    return (
        <div className={`${props.color}Cell`}>
            <Piece {...props.piece} />
        </div>
    )
}

export default Cell;