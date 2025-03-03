import { Colour, PieceType } from "../../pages/shared/enum"
import "./promotion.css"
interface PromotionProps {
    colour : Colour,
    ref : React.RefObject<HTMLDialogElement>
    setPromotionPiece : React.Dispatch<React.SetStateAction<PieceType | null>>
}

const PROMOTION_PIECES = [PieceType.Queen , PieceType.Rook , PieceType.Bishop , PieceType.Knight];
export default function Promotion(props : PromotionProps) {
   return <dialog open className="promotion" ref={props.ref}>
        <form> 
            {PROMOTION_PIECES.map(piece => {
                return <button key={piece} className="promotion-button"> 
                    <img src={`/assets/${props.colour}_${piece}.png`} className="piece"/>
                </button>
            })}
        </form>
    </dialog> 
}