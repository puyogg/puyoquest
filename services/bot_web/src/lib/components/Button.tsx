import { cx } from "class-variance-authority";
import { HTMLAttributes, PropsWithChildren } from "react";

export const buttonBaseStyles =
  "text-white bg-gradient-to-br from-purple-600 to-blue-500 font-medium rounded-lg text-sm px-5 py-2.5 text-center";

export default function Button({
  children,
  ...props
}: PropsWithChildren<HTMLAttributes<HTMLButtonElement>>) {
  return (
    <button {...props} className={cx(props.className, buttonBaseStyles)}>
      {children}
    </button>
  );
}
