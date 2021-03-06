import * as React from 'react';
import Piece from './piece';
import './cell.css'

export const Cell = (props: {
    color: string,
    remote_engine: boolean,
    x: number,
    y: number,
    piece?: {
        idx?: number,
        name?: string
        color?: "WHITE" | "BLACK"
    },
    threatened?: boolean,
    set_board?: any
}) => {
    console.log("piece props: ", props.piece)
    return (
        <div 
            className={`${props.color}Cell ${props.threatened? "threatened": ""}`}
            onClick={
                (ev) => {
                    console.log("FIRED")
                    ev.preventDefault()
                    if (props.remote_engine) {
                        let body = {
                            x: props.x,
                            y: props.y,
                        }
                        console.log("boddddyyy: ", body)

                        fetch("http://localhost:8005/api/play", {
                            method: "POST",
                            mode: "cors",
                            headers: {"Content-Type": "application/json"},
                            body: JSON.stringify(body)
                        })
                        .then(
                            v => v.json()
                                  .then(d => props.set_board(d.board))
                                  .catch(e => {console.log(e)})
                        )
                        .catch(e => {console.log(e)})
                    }
                }
            }
        >
            <Piece {...props.piece} />
        </div>
    )
}

export default Cell;