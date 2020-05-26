#include <iostream>

using namespace std;

class A
{
public:
    int name;
    A(int name);
};
A::A(int name)
{
    cout << "Object is being created" << endl;
    name = name;
}

A test()
{
    A *a = new A(123);

    cout << &(*a).name << endl;
    return *a;
}

A test2()
{
    A a = test();
    cout << &a.name << endl;
    return a;
}

int main()
{
    // A a;
    // a.name = 2;
    // cout << a.name;
    // cout << endl;
    A a = test2();
    cout << &a.name << endl;
    return 0;
}