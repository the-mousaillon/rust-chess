import { height } from "@mui/system"
import * as React from "react"
import Piece from "./piece"

const triggerPromotion = (piece_type: string, color: "WHITE" | "BLACK", set_board: any) => {
    let body;
    try {
        body = JSON.stringify({
            promote_to: piece_type
        })
    }
    catch {
        return
    }
    fetch(
        "http://localhost:8005/api/promote",
        {
            method: "POST",
            mode: "cors",
            body: body,
            headers : {
                "content-type": "application/json"
            }
        }
    ).then(r => {
        r.json().then(b => set_board(b.board))
        .catch(e => console.log(e))
    })
    .catch(e => console.log(e))
}

export const Promotion = (props: {
    color: "WHITE" | "BLACK",
    set_board?: any
}) => {
    return (
        <div style={{
            display: "flex",
            marginLeft: "100px",
            height: "50px",
            width: "560px",
            border: "2px solid black"
        }}>
            <div onClick={() => triggerPromotion("Bishop", props.color, props.set_board)}>
                <Piece name="BISHOP" color={props.color}/>
            </div>
            <div onClick={() => triggerPromotion("Knight", props.color, props.set_board)}>
                <Piece name="KNIGHT" color={props.color} />
            </div>
            <div onClick={() => triggerPromotion("Rook", props.color, props.set_board)}>
                <Piece name="ROOK" color={props.color} />
            </div>
            <div onClick={() => triggerPromotion("Queen", props.color, props.set_board)}>
                <Piece name="QUEEN" color={props.color} />
            </div>
        </div>
    )
}