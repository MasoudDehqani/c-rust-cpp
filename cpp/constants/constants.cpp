#include <iostream>
#define PI 3.14159
#define MUL(a, b) a * b
#define MUL_CORRECT(a, b) (a) * (b)

using namespace std;

// int newFunction() {
//   #define NEW_PI 3.14159 
// }

// int newFunction() {
//   int v {20};
// }

void newFunction() {
  // it would be global in scope, anyway
  #define NEW_PI 3.14159
}

int main() {
  // const double PI {3.14159};
  // const double NEW_PI {3.14159};

  int age {30};
  const int USER_AGE {age};

  cout << newFunction() << endl;

  cout << PI << endl;
  cout << NEW_PI << endl;
  cout << USER_AGE << endl;
  cout << MUL(3, 5) << endl;
  cout << MUL(3+2, 4+1) << endl; // prints 12
  cout << MUL_CORRECT(3+2, 4+1) << endl; // prints 25

  return 0;
}
