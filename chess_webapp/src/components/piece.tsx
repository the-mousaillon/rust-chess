import React, { Component } from 'react';
import Wkn from './pieces/Wkn'
import Bkn from './pieces/Bkn'
import Bb from './pieces/Bb'
import Wb from './pieces/Wb'
import Br from './pieces/Br'
import Wr from './pieces/Wr'
import Bq from './pieces/Bq'
import Wq from './pieces/Wq'
import Bk from './pieces/Bk'
import Wk from './pieces/Wk'
import Bp from './pieces/Bp'
import Wp from './pieces/Wp'
import MoveMarker from './pieces/MoveMarker'



export const Piece = (props: {
    name?: string,
    idx?: number
    color?: "WHITE" | "BLACK"
}) => {
    console.log("props: ", props)
    // king
    if (props.name == "KING" || props.idx == 0) {
        if (props.color == "BLACK") {
            return <Bk />
        }
        else {
            return <Wk />
        }
    }
    // Queen
    else if (props.name == "QUEEN" || props.idx == 1) {
        if (props.color == "BLACK") {
            return <Bq />
        }
        else {
            return <Wq />
        }
    }
    // Rook
    else if (props.name == "ROOK" || props.idx == 2) {
        if (props.color == "BLACK") {
            return <Br />
        }
        else {
            return <Wr />
        }
    }
    // Bishop
    else if (props.name == "BISHOP" || props.idx == 3) {
        if (props.color == "BLACK") {
            return <Bb />
        }
        else {
            return <Wb />
        }
    }
    // Knight
    else if (props.name == "KNIGHT" || props.idx == 4) {
        if (props.color == "BLACK") {
            return <Bkn />
        }
        else {
            return <Wkn />
        }
    }
    // Pawn
    else if (props.name == "PAWN" || props.idx == 5) {
        if (props.color == "BLACK") {
            return <Bp />
        }
        else {
            return <Wp />
        }
    }
    // MoveMarker
    else if (props.name == "MOVE_MARKER" || props.idx == 6) {
        return <MoveMarker />
    }
    // Null
    else {
        return null
    }
}

export default Piece;