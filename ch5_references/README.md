# Chapter 5 - References

## Two Kinds of Reference
1. **Shared Reference**
- Lets you read but not modify its referent.
- It allows you to have many shared references to a particular value at a time as you like.

2. **Mutable Reference/Exclusive Reference**
- Lets you to both read/write its referent.
- You're only allowed to have one Exclusive Reference at a time.

Its **Multiple Reader/Single Writer** rule at a compile time. Also, **References are nonowning pointers**.
