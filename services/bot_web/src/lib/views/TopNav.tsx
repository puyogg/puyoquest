"use client";

import {
  Disclosure,
  DisclosureButton,
  DisclosurePanel,
  Menu,
  MenuButton,
  MenuItem,
  MenuItems,
} from "@headlessui/react";
import { Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { cx } from "class-variance-authority";
import { MetaRole } from "../roles";
import { APIUser } from "discord-api-types/v10";
import Button from "../components/Button";
import { authClient } from "../auth-client";
import Link from "next/link";

interface LinkData {
  name: string;
  href: string;
  current: boolean;
}

interface TopNavProps {
  discordUser?: APIUser;
  roles: MetaRole[];
}

const getPublicLinks = (activePath?: string): LinkData[] => {
  return [
    {
      name: "Cards",
      href: "/",
      current: activePath?.startsWith("/cards") ?? false,
    },
    {
      name: "Bot",
      href: "/bot",
      current: activePath?.startsWith("/bot") ?? false,
    },
  ];
};

const getAuthenticatedLinks = (
  roles: MetaRole[],
  activePath?: string
): LinkData[] => {
  if (roles.includes(MetaRole.USER)) {
    return [
      {
        name: "Decks",
        href: "/decks",
        current: activePath?.startsWith("/decks") ?? false,
      },
      {
        name: "Leaderboards",
        href: "/leaderboards",
        current: activePath?.startsWith("/leaderboards") ?? false,
      },
    ];
  }

  return [];
};

const getAdminLinks = (roles: MetaRole[], activePath?: string): LinkData[] => {
  if (roles.includes(MetaRole.ADMIN)) {
    return [
      {
        name: "Admin",
        href: "/admin",
        current: activePath?.startsWith("/admin") ?? false,
      },
    ];
  }

  return [];
};

export default function TopNav({ discordUser, roles }: TopNavProps) {
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const router = useRouter();

  const links = [
    ...getPublicLinks(pathname),
    ...getAuthenticatedLinks(roles, pathname),
    ...getAdminLinks(roles, pathname),
  ];

  return (
    <Disclosure as="nav" className="bg-gray-800">
      <div className="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
        <div className="relative flex h-16 items-center justify-between">
          {/* Mobile hamburger menu button */}
          <div className="absolute inset-y-0 left-0 flex items-center sm:hidden">
            <DisclosureButton className="group relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:ring-2 focus:ring-white focus:outline-hidden focus:ring-inset">
              <span className="absolute -inset-0.5" />
              <span className="sr-only">Open main menu</span>
              <Bars3Icon
                aria-hidden="true"
                className="block size-6 group-data-open:hidden"
              />
              <XMarkIcon
                aria-hidden="true"
                className="hidden size-6 group-data-open:block"
              />
            </DisclosureButton>
          </div>

          {/* Links (left) */}
          <div className="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start h-full">
            <div className="flex shrink-0 items-center">
              <img
                src="/ex_tetris.png"
                alt="Yotarou - Puyo Puyo Bot"
                className="rounded-full h-10 w-auto"
              />
            </div>

            <div className="hidden sm:ml-6 sm:block">
              <div className="flex space-x-2 h-full items-center">
                {links.map((link) => {
                  return (
                    <a
                      key={link.name}
                      href={link.href}
                      aria-current={link.current ? "page" : undefined}
                      className={cx(
                        link.current
                          ? "bg-gray-900 text-white"
                          : "text-gray-300 hover:bg-gray-700 hover:text-white",
                        "rounded-md px-3 py-2 text-sm font-medium"
                      )}
                    >
                      {link.name}
                    </a>
                  );
                })}
              </div>
            </div>
          </div>

          {/* Profile actions (right) */}
          <div className="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0">
            {/* Profile dropdown */}
            {discordUser && (
              <Menu as="div" className="relative ml-3 text-gray-700">
                <div>
                  <MenuButton className="relative flex rounded-full bg-gray-800 text-sm focus:outline-hidden focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-offset-2 focus-visible:ring-offset-gray-800">
                    <span className="absolute -inset-1.5" />
                    <span className="sr-only">Open user menu</span>
                    <img
                      alt=""
                      src={`https://cdn.discordapp.com/avatars/${discordUser.id}/${discordUser.avatar}.png`}
                      className="size-8 rounded-full"
                    />
                  </MenuButton>
                </div>
                <MenuItems
                  transition
                  className="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black/5 transition focus:outline-hidden data-closed:scale-95 data-closed:transform data-closed:opacity-0 data-enter:duration-100 data-enter:ease-out data-leave:duration-75 data-leave:ease-in"
                >
                  <MenuItem>
                    <Link
                      href="/settings"
                      className="block px-4 py-2 text-sm text-gray-700 data-focus:bg-gray-100 data-focus:outline-hidden"
                    >
                      Settings
                    </Link>
                  </MenuItem>
                  <MenuItem>
                    <a
                      className="block hover:cursor-pointer px-4 py-2 text-sm text-gray-700 data-focus:bg-gray-100 data-focus:outline-hidden"
                      onClick={() => {
                        authClient.signOut({
                          fetchOptions: {
                            onSuccess: () => {
                              router.push("/");
                            },
                          },
                        });
                      }}
                    >
                      Logout
                    </a>
                  </MenuItem>
                </MenuItems>
              </Menu>
            )}
            {/* Sign In */}
            {!discordUser && (
              <Button
                onClick={async () => {
                  authClient.signIn.social({
                    provider: "discord",
                    callbackURL: `${pathname}?${searchParams.toString()}`,
                  });
                }}
              >
                Login
              </Button>
            )}
          </div>
        </div>
      </div>

      <DisclosurePanel className="sm:hidden">
        <div className="space-y-1 px-2 pt-2 pb-3">
          {links.map((link) => {
            return (
              <DisclosureButton
                key={link.name}
                as="a"
                href={link.href}
                aria-current={link.current ? "page" : undefined}
                className={cx(
                  link.current
                    ? "bg-gray-900 text-white"
                    : "text-gray-300 hover:bg-gray-700 hover:text-white",
                  "block rounded-md px-3 py-2 text-base font-medium"
                )}
              >
                {link.name}
              </DisclosureButton>
            );
          })}
        </div>
      </DisclosurePanel>
    </Disclosure>
  );
}
