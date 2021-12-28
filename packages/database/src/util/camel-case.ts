import * as _ from 'lodash';

export function camelCase<T>(obj: object): T {
  const camelCaseObj: Record<string, unknown> = {};

  Object.entries(obj).forEach(([key, value]) => {
    const camelKey = _.camelCase(key);
    Object.assign(camelCaseObj, { [camelKey]: value });
  });

  return camelCaseObj as T;
}
