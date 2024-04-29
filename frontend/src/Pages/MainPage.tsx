import { useSelector } from "react-redux";
import AccountTable from "../Components/AccountTable";
import ActionButton from "../UI/ActionButton";
import styles from "./MainPage.module.scss";
import { FC } from "react";
import { RootState } from "../state/store";

const MainPage: FC = () => {
  const checked_items = useSelector<RootState, string[]>(
    (state) => state.checked_items.items
  );

  return (
    <section className={styles.main_page}>
      <div className={styles.content}>
        <div className={styles.header_container}>
          <h2 className={styles.h2}>Accounts</h2>
          <div className={styles.action_buttons}>
            {checked_items.length === 0 ? (
              <>
                <ActionButton kind="new_transaction" />
                <ActionButton kind="new_account" />
              </>
            ) : (
              <>
                <ActionButton kind="open_credit" />
                <ActionButton kind="delete_account" />
              </>
            )}
          </div>
        </div>
        <AccountTable />
      </div>
    </section>
  );
};

export default MainPage;
