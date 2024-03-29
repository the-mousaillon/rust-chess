import { Button } from "@mui/material"
import * as React from "react"
import { ChessBoard } from "./chessboard"

interface GameState {
    current_player: String,
    turn: number,
    board?: any,
    board_history: any[]
}


const continuous_ai_play = (set_game_state) => {
    fetch("http://localhost:8005/api/play", {
        method: "POST",
        mode: "cors",
        headers: {
            "content-type": "application/json"
        },
        body: JSON.stringify({x: 0, y: 0})
    })
    .then(
        v => v.json()
            .then(d => {
                console.log("game state", d)
                set_game_state(d)
                setTimeout(() => {
                    continuous_ai_play(set_game_state)
                }, 500);
                
            })
            .catch(e => {console.log(e)})
    )
    .catch(e => {console.log(e)})
}

function useKeyPressed(key: string) {
    let [key_pressed, set_key_pressed] = React.useState(false)
    const downHandler = (ev: any) => {
        if (ev.key === key) {
            set_key_pressed(true);
        }
    }
      // If released key is our target key then set to false
      const upHandler = (ev: any) => {
        if (ev.key === key) {
            set_key_pressed(false);
        }
    }

    React.useEffect(() => {
        window.addEventListener("keydown", downHandler)
        window.addEventListener("keyup", upHandler)
        return () => {
            window.removeEventListener("keydown", downHandler)
            window.removeEventListener("keyup", upHandler)      
        }
    }, [])
    return key_pressed
}


export const Game = (props: {}) => {
    let [game_state, set_game_state] = React.useState()
    let [offset, set_offset] = React.useState(0)
    let [mode, set_mode] = React.useState()
    let prev_board = useKeyPressed("ArrowLeft")
    let next_board = useKeyPressed("ArrowRight")
    React.useEffect(() => set_mode("Ai vs Ai"), [])
    React.useEffect(() => {
        try {
            let body;
            if (mode == "Ai vs Ai") {
                body = JSON.stringify({Setup:{AiVsAi:["MiniMaxAi", "MiniMaxAi", 3, 3]}})
            }
            else if (mode == "Ai vs Player") {
                body = JSON.stringify({Setup:{PlayerVsAi:["White", "MiniMaxAi"]}})
            }
            fetch("http://localhost:8005/api/set_play_mode", {
                method: "POST",
                mode: "cors",
                headers: {
                    "content-type": "application/json"
                },
                body: body
            })
            .then(
                v => v.json()
                    .then(d => {
                        console.log("game state", d)
                        set_game_state(d)
                    })
                    .catch(e => {console.log(e)})
            )
            .catch(e => {console.log(e)})
        }
        catch (e) {
            console.log(e)
        }
    }, [mode])
    console.log("true game state: ", game_state)
    console.log("offset: ", offset, "next_board", next_board, "prev_board", prev_board)

    React.useEffect(() => {
        if (prev_board) {
            let new_offset = offset + 1
            if (new_offset <= game_state.turn) {
                set_offset(new_offset)
            }
        }
        else if (next_board) {
            let new_offset = offset - 1
            if (new_offset >= 0) {
                set_offset(new_offset)
            }
        }
    }, [prev_board, next_board])

    let board

    if (offset == 0) {
        board = game_state && game_state.board
    }
    else {
        board = game_state.board_history[game_state.turn - offset]
    }
    return (
        <div>
            <Button variant="outlined" onClick={() => continuous_ai_play(set_game_state)}>Ai play</Button>
            <Button variant="outlined" onClick={() => set_mode("Ai vs Ai")}>Ai VS Ai</Button>
            <Button variant="outlined" onClick={() => set_mode("Ai vs Player")}>Ai VS Player</Button>
            <span>Mode: {mode}</span>
            <ChessBoard 
                board={board}
                remote_engine={true}
                set_game_state={set_game_state}
            />
        </div>
    )
}