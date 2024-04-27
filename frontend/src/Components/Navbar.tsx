import { NavLink } from "react-router-dom";
import styles from "./Navbar.module.scss";
import "../general_styles.scss";
import { FC } from "react";

const Navbar: FC = () => {
  return (
    <nav className={styles.navbar}>
      <div className={styles.content}>
        <div className={styles.header_container}>
          <h1 className={styles.h1}>
            Bank <span>client</span>
          </h1>
          <div className={styles.info_container}>
            <p className={styles.info_unit}>
              Store card: <span>8174505736614003</span>
            </p>
            <p className={styles.info_unit}>
              Store balance: <span>100_000</span>
            </p>
            <p className={styles.info_unit}>
              Bank emission: <span>432_129</span>
            </p>
          </div>
        </div>
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
      </div>
    </nav>
  );
};

export default Navbar;
