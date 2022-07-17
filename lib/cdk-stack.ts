import { Stack, StackProps } from 'aws-cdk-lib';
import { FunctionUrl, FunctionUrlAuthType } from 'aws-cdk-lib/aws-lambda'
import { Construct } from 'constructs';
import { CfnOutput } from 'aws-cdk-lib';
import * as s3 from 'aws-cdk-lib/aws-s3';
import { RustFunction, Settings } from 'rust.aws-cdk-lambda';

const directory = '/home/pfes/brein_rs'

export class CdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const bucket = new s3.Bucket(this, 'recipe_bucket', {
      publicReadAccess: true,
    });

    Settings.BUILD_ENVIRONMENT = {
      GET_RECIPES_BUCKET_NAME: bucket.bucketName 
    }

    const get_recipes = new RustFunction(this, 'get_recipes_function', {
      directory,
      bin: 'get_recipes',
      features: ['lambda'],
    });

    const get_recipes_url = new FunctionUrl(this, 'get_recipes_url', {
      function: get_recipes,
      authType: FunctionUrlAuthType.NONE
    });

    const lambda = new RustFunction(this, 'brein_lambda', {
      directory,
      bin: 'lambda',
      features: ['lambda']
    });

    const url = new FunctionUrl(this, 'brein_function_url', {
      function: lambda,
      authType: FunctionUrlAuthType.NONE
    });

    new CfnOutput(this, "brein function url", {
      value: url.url,
    });

    new CfnOutput(this, "recipe bucket arn", {
      value: bucket.bucketArn
    });

    new CfnOutput(this, "get recipes url", {
      value: get_recipes_url.url,
    });
  }
}
