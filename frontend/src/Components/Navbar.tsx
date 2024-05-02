import styles from "./Navbar.module.scss";
import "../general_styles.scss";
import { FC, useEffect, useRef, useState } from "react";
import NavInfo from "./NavInfo";
import NavLinks from "./NavLinks";
import { IoMenu } from "react-icons/io5";
import { FaXmark } from "react-icons/fa6";

const Navbar: FC = () => {
  const [mobile_menu_open, set_mobile_menu_open] = useState(false);
  const mobile_menu_ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handle_click_outside = (e: MouseEvent) => {
      if (mobile_menu_ref.current) {
        if (!mobile_menu_ref.current.contains(e.target as Node)) {
          set_mobile_menu_open(false);
        }
      }
    };

    document.addEventListener("mousedown", handle_click_outside);

    return () => {
      document.removeEventListener("mousedown", handle_click_outside);
    };
  }, []);

  return (
    <nav className={styles.navbar}>
      <div className={styles.content}>
        <div className={styles.header_container}>
          <h1 className={styles.h1}>
            Bank <span>client</span>
          </h1>
          <div className={styles.navinfo_desktop}>
            <NavInfo />
          </div>
          <div
            className={styles.mobile_icons_container}
            onClick={() => set_mobile_menu_open(!mobile_menu_open)}
          >
            {mobile_menu_open ? (
              <FaXmark className={styles.menu_icon} />
            ) : (
              <IoMenu className={styles.menu_icon} />
            )}
          </div>
        </div>
        <div className={styles.navlinks_desktop}>
          <NavLinks />
        </div>
      </div>
      <div
        ref={mobile_menu_ref}
        className={`${styles.mobile_menu} ${
          mobile_menu_open && styles.mobile_menu_open
        }`}
      >
        <NavLinks />
        <hr className={styles.divider} />
        <NavInfo />
      </div>
    </nav>
  );
};

export default Navbar;
