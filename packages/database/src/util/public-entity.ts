/**
 * Replace optional properties (i.e. keys with type
 * T[K] | undefined) with T[K] | null.
 * Then, make all optional properties required.
 */
export type PublicEntity<T> = {
  [K in keyof T]-?: undefined extends T[K] ? Exclude<T[K], undefined> | null : T[K];
};
