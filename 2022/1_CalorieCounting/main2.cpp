#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>
using namespace std;

int main()
{

  int sum = 0;
  int max = 0;
  
  vector<int> nums;
  string line;
  while(std::cin)
  {
    getline(std::cin,line);
    
    if(line=="")
    {
      if(sum>max)
        nums.push_back(sum); 
      sum=0;
      continue;
    }
    sum += stoi(line);
  }
  
  if(sum>max)
    nums.push_back(sum); 
  sort(nums.begin(),nums.end(), greater<int>());
  
  cout << nums[0]+nums[1]+nums[2] << endl;
}