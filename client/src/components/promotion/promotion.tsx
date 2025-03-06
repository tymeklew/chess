import { Colour, PieceType } from "../../pages/shared/enum"
import "./promotion.css"
interface PromotionProps {
    colour : Colour,
    setPromotionPiece : React.Dispatch<React.SetStateAction<PieceType | null>>
}

const PROMOTION_PIECES = [PieceType.Queen , PieceType.Rook , PieceType.Bishop , PieceType.Knight];
export default function Promotion(props : PromotionProps) {
    function handleSubmit() {

    }
   return <dialog className="promotion" open>
        <form onSubmit={handleSubmit}> 
            {PROMOTION_PIECES.map(piece => {
                return <button key={piece} className="promotion-button" type="submit"> 
                    <img src={`/assets/${props.colour}_${piece}.png`} className="piece"/>
                </button>
            })}
        </form>
    </dialog> 
}