import "./board.css";

const ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
export default function Board() {
  return (
    <div className="board">
      {Array.from({ length: 64 }).map((_, index) => {
        return (
          <div
            key={index}
            className={`tile tile-${(index + Math.floor(index / 8)) % 2 === 0 ? "light" : "dark"}`}
          >
            {index % 8 == 7 ? (
              <p className="upper-number numbering">
                {" "}
                {Math.floor(9 - index / 8)}{" "}
              </p>
            ) : (
              <> </>
            )}
            {index / 8 >= 7 ? (
              <p className="lower-letter numbering">{ALPHABET[index % 8]}</p>
            ) : (
              <> </>
            )}
          </div>
        );
      })}
    </div>
  );
}
