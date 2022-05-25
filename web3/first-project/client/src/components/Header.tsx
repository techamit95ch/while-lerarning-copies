type props = {
  children: React.ReactNode;
};
const Header = ({ children }: props) => {
  return <header className="headerConatiner">{children}</header>;
};

export default Header;
