import { Link } from "react-router-dom";
import "./header.css";
export default function Header() {
  return (
    <header className="header">
      <div className="logo">
        <h1> Lorem Ipsum </h1>
      </div>
      <nav className="navigation">
        <ul>
          <li>
            <Link to="play" className="nav-link">
              Play
            </Link>
          </li>
          <li>
            {" "}
            <Link to="login" className="nav-link">
              Login
            </Link>
          </li>
          <li>
            {" "}
            <Link to="signup" className="nav-link">
              Sign Up
            </Link>
          </li>
        </ul>
      </nav>
    </header>
  );
}
