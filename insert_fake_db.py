from surrealdb import Surreal
import psycopg2
import random
import hashlib
from datetime import datetime
import uuid

async def main():
    def generate_username(name):
        return name.lower().replace(" ", "") + str(random.randint(1, 999))

    # Function to generate a random phone number
    def generate_phone_number():
        return f"+1-{random.randint(100, 999)}-{random.randint(100, 999)}-{random.randint(1000, 9999)}"
    

    # Function to generate a random user
    def generate_random_user():
        name = "User " + str(random.randint(1, 1000))
        username = generate_username(name)
        mob_phone = generate_phone_number()
        password = hashlib.sha256(str(random.random()).encode()).hexdigest()
        access_level = random.choice(["trainee", "employee", "manager", "admin"])
        status = random.choice(["active", "inactive"])
        a_created = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        return (name, username, mob_phone, password, access_level, status, a_created)

    async with Surreal("ws://localhost:8000/rpc") as db:
        print("Connecting to db...")
        await db.signin({"user": "arturs", "pass": "arturs"})
        await db.use("test", "test")
        for _ in range(1000):
            user_data = generate_random_user()
            user_dict = {
                "name": user_data[0],
                "username": user_data[1],
                "mob_phone": user_data[2],
                "password": user_data[3],
                "access_level": user_data[4],
                "status": user_data[5],
                "a_created": user_data[6]
            }

        await db.create("user", user_dict)
        print("User created:", user_dict)


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())

# from surrealdb import Surreal
# import psycopg2
# import random
# import hashlib
# from datetime import datetime
# import uuid

# async def main():

#     def generate_username(name):
#         return name.lower().replace(" ", "") + str(random.randint(1, 999))


#     def generate_phone_number():
#         return f"+1-{random.randint(100, 999)}-{random.randint(100, 999)}-{random.randint(1000, 9999)}"
    
#     """Example of how to use the SurrealDB client."""
#     async with Surreal("ws://localhost:8000/rpc") as db:
#         await db.signin({"user": "root", "pass": "root"})
#         await db.use("test", "test")
#         for _ in range(1000):
#             await db.create(
#                 "staff_users",
#                 {
#                     "name": "User " + (str(random.randint(1, 1000)) + str(random.randint(1, 1000)) + str(random.randint(1, 1000)) + str(random.randint(1, 1000))),
#                     "username": "User " + (str(random.randint(1, 1000)) + str(random.randint(1, 1000)) + str(random.randint(1, 1000)) + str(random.randint(1, 1000))),
#                     "mob_phone": generate_phone_number(),
#                     "passwd": hashlib.sha256(str(random.random()).encode()).hexdigest(),
#                     "access_level": random.choice(["trainee", "employee", "manager", "admin"]),
#                     "status": random.choice(["active", "inactive"]),
#                     ##convert datetime to string
#                     # "a_created": datetime.now()
#                 },
#             )

# if __name__ == "__main__":
#     import asyncio

#     asyncio.run(main())