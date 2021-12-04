#include <stdio.h>

#define NUM_CUPS 1000000
#define NUM_ROUNDS 10000000
unsigned int cups[NUM_CUPS+1];
unsigned int initial_cups[] = {8, 5, 3, 1, 9, 2, 6, 4, 7};
//int initial_cups[] = {3, 8, 9, 1, 2, 5, 4, 6, 7};
unsigned int popped_cups[3];

int main()
{
    const int initial_len = sizeof(initial_cups)/sizeof(initial_cups[0]);    
    cups[0] = 0;
    for(int i=1; i<NUM_CUPS; ++i) {
        if(i < initial_len)
        {
            cups[initial_cups[i-1]] = initial_cups[i];
        }
        else if(i == initial_len)
        {
            cups[initial_cups[i-1]] = i + 1;
        }
        else
        {
            cups[i] = i + 1;
        }
    }
    cups[NUM_CUPS] = initial_cups[0]; // Circular list

    int current_cup = initial_cups[0];
    for(int g=0; g<NUM_ROUNDS; ++g)
    {
        int cur = cups[current_cup];
        for(int i=0; i<3; ++i)
        {
            popped_cups[i] = cur;
            cur = cups[cur];
        }
        cups[current_cup] = cur;

        int label = current_cup - 1;
        while (1)
        {
            if(label < 1) label = NUM_CUPS;
            char found = 0;
            for(int i=0; i<3; ++i)
            {
                if (popped_cups[i] == label)
                {
                    found = 1;
                    break;    
                }
            }
            if(!found) break;
            label--;
        }
        int next = cups[label];
        cups[label] = popped_cups[0];
        cups[popped_cups[2]] = next;

        current_cup = cups[current_cup];
    }

    long int val1 = cups[1];
    long int val2 = cups[val1];

    //printf("%ld\n", val1);
    //printf("%ld\n", val2);
    printf("%ld\n", val1 * val2);
    /*
    for(int i=0; i<100; ++i) {
        printf("%d=%d ", i, cups[i]);
    }
    printf("\n");
    for(int i=NUM_CUPS-100; i<=NUM_CUPS; ++i) {
        printf("%d=%d ", i, cups[i]);
    }
    printf("\n");
    */
}