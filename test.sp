static_assert(true, "This is an assertion error");
assert(true, "This is an assertion error");
using __intrinsics__.Handle;


stock float operator++(float oper)
{
	return oper + 1.0;
}


native float operator*(float oper1, float oper2) = FloatMul;
native float operator/(float oper1, float oper2) = FloatDiv;


typedef SQLTxnFailure = function void(Database db, any data, int numQueries, const char[] error, int failIndex, any[] queryData);


typeset EventHook {
// comment
	function Action(Event event, const char[] name, bool dontBroadcast);
// comment
	function void(Event event, const char[] name, bool dontBroadcast);
};


enum FOO(<<= 1.0)
{
	BIT1 = 1,
	BIT2 = 4,
	BIT3,
	BIT4,
}


void OnPluginStart()
{
	if(true)
	{
		te++;
	}
	else if(true)
	{
		te++;
	}
	else
	{
		te++;
	}
	if(true)
		te++;
	else if(true)
		te++;
	else
		te++;
	while(true)
	{
		i++;
	}
	while(true)
		i++;
	for(int i = 0; i <= MaxClients; i++)
	{
		i++;
	}
	for(int i = 0; i <= MaxClients; i++)
		i++;
	do
	{
		o++
	}
	while(true)

	switch(1)
	{
		case 1:
			true;
		case 2:
		{
			hello;
		}
		default:
			true;
	}

	int foo;
	foo = test(1, 2);

	delete foo;
}


enum struct WEAPONS_ENUM
{
	int KNIFE;
	int GLOCK;
	int HKP2000;


	int GetData(int[] data)
	{
		data[0] = this.KNIFE;
		data[1] = this.GLOCK;
		data[2] = this.HKP2000;
	}


	void Reset()
	{
		this.KNIFE = 0;
		this.GLOCK = 0;
		this.HKP2000 = 0;
	}


	void AddKill(int num)
	{
		switch(num)
		{
			case 0:
			{
				this.KNIFE++;
			}
			case 1:
			{
				this.GLOCK++;
			}
			case 2:
			{
				this.HKP2000++;
			}
		}
	}
}

