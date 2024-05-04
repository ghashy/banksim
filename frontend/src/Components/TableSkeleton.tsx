import styles from "./AccountTable.module.scss";
import { FC } from "react";

const TableSkeleton: FC = () => {
  return (
    <div className={styles.skeleton_container}>
      <div className={`${styles.skeleton_block} ${styles.block1}`}></div>
      <div className={`${styles.skeleton_block} ${styles.block2}`}></div>
      <div className={`${styles.skeleton_block} ${styles.block3}`}></div>
      <div className={`${styles.skeleton_block} ${styles.block4}`}></div>
      <div className={`${styles.skeleton_block} ${styles.block5}`}></div>
    </div>
  );
};

export default TableSkeleton;
