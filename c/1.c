#include <stdio.h>
#include <stdlib.h>

// Function definition
int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
    int* result = malloc(2 * sizeof(int));
    for (int i = 0; i < numsSize; i++)
        for (int j = 0; j < numsSize; j++)
            if (i != j && nums[i] + nums[j] == target) {
                result[0]   = i;
                result[1]   = j;
                *returnSize = 2;
                return result;
            }
    exit(1);
}

// Function to run a single test case
void runTest(int* nums, int numsSize, int target, int* expected) {
    int  returnSize;
    int* result = twoSum(nums, numsSize, target, &returnSize);
    if (result[0] == expected[0] && result[1] == expected[1])
        printf("Test passed\n");
    else
        printf(
            "Test failed: expected [%d, %d], got [%d, %d]\n",
            expected[0],
            expected[1],
            result[0],
            result[1]
        );
    free(result);
}

// Main function to run all tests
int main() {
    // Test case 1
    int nums1[]     = {2, 7, 11, 15};
    int target1     = 9;
    int expected1[] = {0, 1};
    runTest(nums1, 4, target1, expected1);

    // Test case 2
    int nums2[]     = {3, 2, 4};
    int target2     = 6;
    int expected2[] = {1, 2};
    runTest(nums2, 3, target2, expected2);

    // Test case 3
    int nums3[]     = {3, 3};
    int target3     = 6;
    int expected3[] = {0, 1};
    runTest(nums3, 2, target3, expected3);

    return 0;
}
