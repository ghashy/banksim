import styles from "./Navbar.module.scss";
import { useSelector } from "react-redux";
import { FC } from "react";
import { RootState } from "../state/store";
import { IStoreInfo } from "../types";

const NavInfo: FC = () => {
  const store_info = useSelector<RootState, IStoreInfo>(
    (state) => state.store_info
  );

  return (
    <div className={styles.info_container}>
      <p className={styles.info_unit}>
        Store card:{" "}
        {!store_info.card.is_loading ? (
          <span>{store_info.card.content}</span>
        ) : (
          <>
            <span className={styles.dash1}>-</span>
            <span className={styles.dash2}>-</span>
          </>
        )}
      </p>
      <p className={styles.info_unit}>
        Store balance:{" "}
        {!store_info.balance.is_loading ? (
          <span>{store_info.balance.content}</span>
        ) : (
          <>
            <span className={styles.dash1}>-</span>
            <span className={styles.dash2}>-</span>
          </>
        )}
      </p>
      <p className={styles.info_unit}>
        Bank emission:{" "}
        {!store_info.emission.is_loading ? (
          <span>{store_info.emission.content}</span>
        ) : (
          <>
            <span className={styles.dash1}>-</span>
            <span className={styles.dash2}>-</span>
          </>
        )}
      </p>
    </div>
  );
};

export default NavInfo;
