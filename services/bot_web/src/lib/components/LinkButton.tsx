import { cx } from "class-variance-authority";
import Link from "next/link";
import { HTMLAttributes, PropsWithChildren } from "react";
import { buttonBaseStyles } from "./Button";

export default function LinkButton({
  href,
  children,
  ...props
}: PropsWithChildren<HTMLAttributes<HTMLAnchorElement> & { href: string }>) {
  return (
    <Link
      {...props}
      href={href}
      className={cx(props.className, buttonBaseStyles)}
    >
      {children}
    </Link>
  );
}
