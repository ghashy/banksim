import styles from "./Navbar.module.scss";
import { NavLink } from "react-router-dom";
import { FC } from "react";

const NavLinks: FC = () => {
  return (
    <div className={styles.nav_links}>
      <NavLink
        to="/"
        className={styles.nav_link}
      >
        Commands
      </NavLink>
      <NavLink
        to="transactions"
        className={styles.nav_link}
      >
        Transactions
      </NavLink>
      <NavLink
        to="logs"
        className={styles.nav_link}
      >
        Logs
      </NavLink>
      <NavLink
        to="about"
        className={styles.nav_link}
      >
        About
      </NavLink>
    </div>
  );
};

export default NavLinks;
