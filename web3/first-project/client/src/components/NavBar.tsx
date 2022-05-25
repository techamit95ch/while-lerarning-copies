import { HiMenuAlt4 } from "react-icons/hi";
import { AiOutlineClose } from "react-icons/ai";
import logo from "../images/logo.png";
import { useState } from "react";

interface navbarProps {
  title?: string;
  classProps?: string;
}
const NavBarItem = ({ title, classProps }: navbarProps) => {
  return <li className={`navItem ${classProps}`}>{title}</li>;
};
const NavBar = () => {
  const [menu = false, setMenu] = useState<Boolean>();
  return (
    <nav className="navContainer">
      <div className="navWrapper">
        <img src={logo} alt="logo" className="navImgLogo" />
      </div>
      <ul className="navUl">
        {["Market", "Exchange", "Tutorials", "Wallets"].map((item, index) => (
          <NavBarItem title={item} key={index} />
        ))}
        <li className="navLoginButton">Login</li>
      </ul>
      <div className="navMenu">
        {!menu && <HiMenuAlt4 fontSize={28} onClick={() => setMenu(true)} />}
        {menu && (
          <ul className="navMenuUl">
            <li className="navMenuLi">
              <AiOutlineClose onClick={() => setMenu(false)} />
            </li>
            {["Market", "Exchange", "Tutorials", "Wallets"].map(
              (item, index) => (
                <NavBarItem
                  title={item}
                  key={index}
                  classProps="navClassProps"
                />
              )
            )}
          </ul>
        )}
      </div>
    </nav>
  );
};
export default NavBar;
