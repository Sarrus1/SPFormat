#include <sourcemod>	/** */
#tryinclude "foo"	// foo
#define TEST "Hello"	// foo

// Variadic preprocessor function doesn't actually require anything significant, it seems.
#define PRINTCHATV(%0, %1, %2) ( PrintColorChat( %0, %1, %2 ) )

#undef PRINTCHATV	//foo

#endinput

#if TEST

#elseif TEST
#else
#endif

#error An error
#warning A warning
#assert 1==1
#pragma newdecls required

static_assert(true, "This is an assertion error");	// foo
assert(true, "This is an assertion error");	// test

using __intrinsics__.Handle;

int number;
float pointnumber;

char character;
bool boolean;
float vec[3];
bool active[MAXPLAYERS + 1];
Action action;
Action action1, action2;

new a;
const a;
a;
new Float: b = 0.23;
Float: b = 0.23;
new a = 0 + 1;
new bool: c;
new _: d = 2, Float: e, bool: f = true, String: g = 'c';
new Action: ac = INVALID_HANDLE;
Action: ac = INVALID_HANDLE;
vec[MAXPLAYERS + 1];
vec[];


stock float operator++(float oper)
{
	return oper + 1.0;
}


native float operator*(float oper1, float oper2) = FloatMul;
native float operator/(float oper1, float oper2) = FloatDiv;


typedef SQLTxnFailure = function void (Database db, any data, int numQueries, const char[] error, int failIndex, any[] queryData);


functag SrvCmd Action: public(args);


funcenum Timer
{
	// comment
	Action: public (Handle:Timer, Handle:hndl),
	// comment
	Action: public (Handle:timer),
};


typeset EventHook
{
	// comment
	function Action (Event event, const char[] name, bool dontBroadcast);
	// comment
	function void (Event event, const char[] name, bool dontBroadcast);
};


enum FOO(<<= 1.0)
{
	BIT1 = 1,
	BIT2 = 4,
	BIT3,
	BIT4,
}


methodmap EmbedFooter < JSONObject
{
	/**
	 * Constructor for the EmbedFooter methodmap.
	 * 
	 * @param text			Text of the footer.
	 * @return					Returns the EmbedFooter.
	 */
	public EmbedFooter(const char[] text = "")
	{
		JSONObject jsonObject = new JSONObject();
		jsonObject.SetString("text", text);
		return view_as<EmbedFooter>(jsonObject);
	}
	/**
	 * Retrieve the text of the footer.
	 * 
	 * @param buffer				String buffer to store value.
	 * @param maxlength			Maximum length of the string buffer.
	 * @return							True on success. False otherwise.
	 */
	public bool GetText(char[] buffer, int maxlength)
	{
		return this.GetString("text", buffer, maxlength);
	}
};


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

