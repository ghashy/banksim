import styles from "./NotFoundPage.module.scss";
import { FC } from "react";

const NotFoundPage: FC = () => {
  return (
    <section className={styles.not_found_page}>
      <div className={styles.content}>Page not found</div>
    </section>
  );
};

export default NotFoundPage;
