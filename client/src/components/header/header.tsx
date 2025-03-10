import { Link } from "react-router-dom";
import "./header.css";
import { useAuth } from "../../pages/shared/context";
export default function Header() {
  const auth = useAuth();
  const loggedIn = auth ? auth.loggedIn : false;
  return (
    <header className="header">
      <div className="logo">
        <h1> Chess </h1>
      </div>

      <nav className="navigation">

        <ul>
          {loggedIn ? (
            <>
          <li>
            <Link to="play" className="nav-link">
              Play
            </Link>
          </li>
          <li>
            <Link to="/friends" className="nav-link">
              Friends
            </Link>
          </li>
          <li onClick={auth?.logOut}>
            Log Out
          </li>
          </>
        ) : (<>
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
          </>)}
        </ul>
      </nav>
    </header>
  );
}
