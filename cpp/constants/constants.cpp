#include <iostream>

using namespace std;

int main() {
  const double PI {3.14159};

  int age {30};
  const int USER_AGE {age};

  cout << PI << endl;
  cout << USER_AGE << endl;

  return 0;
}
