import { findIndex, max } from "lodash";

class OrderService {
    //TODO Write unit test
    public getFollowingOrderNumber(items: { order: number; [key: string]: any }[]): number {
        const highestOrderNumberInItems = max(items.map((item) => item.order));

        return highestOrderNumberInItems != undefined ? highestOrderNumberInItems + 1 : 0;
    }

    public getItemListWithUpdatedOrders<T>(items: { order: number; [key: string]: any }[]): T[] {
        return items.map(
            (item) =>
                ({
                    ...item,
                    order: findIndex(items, item)
                }) as T
        );
    }
}

export default new OrderService();
