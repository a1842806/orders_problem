USE VSEC_DB;

-- 1
SELECT status, COUNT(*) AS order_count
FROM Orders
GROUP BY status;

-- 2
SELECT user_id, COUNT(*) AS order_count
FROM Orders
GROUP BY user_id
ORDER BY order_count DESC
LIMIT 10;

-- 3
SELECT COUNT(DISTINCT user_id) AS unique_clients
FROM Orders;

-- 4
SELECT user_id, COUNT(*) AS closed_order_count
FROM Orders
WHERE status = 'CLOSED'
GROUP BY user_id
ORDER BY closed_order_count DESC
LIMIT 1;